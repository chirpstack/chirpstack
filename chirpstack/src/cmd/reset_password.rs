use anyhow::{Context, Result};

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
/// * `password_file` - Path to file containing the new password (use "-" for stdin, omit for interactive prompt)
pub async fn run(email: &str, password_file: &Option<String>) -> Result<()> {
    let password = get_password(password_file).context("Failed to get password")?;

    // Validate password strength (NIST 800-63b guidelines)
    user::validate_password_strength(&password)?;

    // Setup database connection
    crate::storage::setup()
        .await
        .context("Failed to setup storage")?;

    // Reset the password
    let user = user::reset_password_by_email(email, &password)
        .await
        .with_context(|| format!("Failed to reset password for user: {}", email))?;

    println!("Password reset for user: {}", user.email);

    Ok(())
}

fn get_password(password_file: &Option<String>) -> Result<String> {
    // File input (including stdin with "-")
    if let Some(path) = password_file {
        if path == "-" {
            // Read from stdin when "-" is specified
            let input = rpassword::read_password().context("Failed to read password from stdin")?;
            return Ok(input);
        }
        let pw = std::fs::read_to_string(path.as_str()).context("Failed to read password file")?;
        // Trim only trailing newline/carriage return (common when echo "pass" > file)
        let trimmed = pw.trim_end_matches(&['\n', '\r'][..]);
        return Ok(trimmed.to_string());
    }

    // Interactive prompt
    let password = rpassword::prompt_password("New password: ")?;
    let confirm = rpassword::prompt_password("Confirm password: ")?;

    if password != confirm {
        anyhow::bail!("Passwords do not match");
    }

    Ok(password)
}
