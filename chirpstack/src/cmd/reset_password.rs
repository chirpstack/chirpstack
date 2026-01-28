use anyhow::{Context, Result};
use tracing::info;

use crate::config;
use crate::storage::user;

/// Reset a user's password from the command-line.
///
/// This command enables password reset without API access, useful for:
/// - Initial setup of fresh installations
/// - Automated deployment scripts
/// - Recovery from forgotten passwords
///
/// # Arguments
///
/// * `email` - User email address (unique identifier)
/// * `password_file` - Path to file containing the new password (use "-" for stdin)
/// * `stdin` - Read password from stdin
pub async fn run(email: &str, password_file: &Option<String>, stdin: bool) -> Result<()> {
    let password = get_password(password_file, stdin).context("Failed to get password")?;

    // Validate password strength (NIST 800-63b guidelines)
    validate_password_strength(&password)?;

    // Load configuration
    let _conf = config::get();

    // Setup database connection
    crate::storage::setup()
        .await
        .context("Failed to setup storage")?;

    // Reset the password
    let user = user::reset_password_by_email(email, &password)
        .await
        .with_context(|| format!("Failed to reset password for user: {}", email))?;

    info!(email = %user.email, "Password reset successfully");

    println!("Password reset for user: {}", user.email);

    Ok(())
}

fn get_password(password_file: &Option<String>, stdin: bool) -> Result<String> {
    // Stdin takes precedence
    if stdin {
        let input = rpassword::read_password().context("Failed to read password from stdin")?;
        return Ok(input);
    }

    // File input - only trim trailing newlines that commonly come from echo/file writes
    if let Some(path) = password_file {
        if path != "-" {
            let pw =
                std::fs::read_to_string(path.as_str()).context("Failed to read password file")?;
            // Trim only trailing newline/carriage return (common when echo "pass" > file)
            let trimmed = pw.trim_end_matches(&['\n', '\r'][..]);
            return Ok(trimmed.to_string());
        }
    }

    // Interactive prompt
    let password = rpassword::prompt_password("New password: ")?;
    let confirm = rpassword::prompt_password("Confirm password: ")?;

    if password != confirm {
        anyhow::bail!("Passwords do not match");
    }

    Ok(password)
}

/// Validate password against security requirements.
///
/// Follows NIST 800-63b guidelines:
/// - Minimum 8 characters
/// - No complexity requirements (users choose better passwords)
/// - Maximum length to prevent DoS attacks
fn validate_password_strength(password: &str) -> Result<()> {
    if password.len() < 8 {
        anyhow::bail!("Password must be at least 8 characters");
    }

    // NIST guidelines suggest NOT requiring special characters,
    // uppercase, lowercase, numbers, etc. as this leads to weaker passwords.
    // However, a maximum length prevents DoS attacks.
    if password.len() > 128 {
        anyhow::bail!("Password must not exceed 128 characters");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password_strength_too_short() {
        assert!(validate_password_strength("abc").is_err());
        assert!(validate_password_strength("1234567").is_err());
    }

    #[test]
    fn test_validate_password_strength_valid() {
        assert!(validate_password_strength("validpassword123").is_ok());
        assert!(validate_password_strength("MySecureP@ssw0rd!").is_ok());
    }

    #[test]
    fn test_validate_password_strength_too_long() {
        assert!(validate_password_strength(&"a".repeat(129)).is_err());
        assert!(validate_password_strength(&"a".repeat(200)).is_err());
    }

    #[test]
    fn test_validate_password_strength_boundary() {
        // Exactly 8 characters should pass
        assert!(validate_password_strength("12345678").is_ok());
        // Exactly 128 characters should pass
        assert!(validate_password_strength(&"a".repeat(128)).is_ok());
    }
}
