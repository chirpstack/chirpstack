use anyhow::{Context, Result};

use crate::storage::{self, user};

// This command sets an user password, either by prompting the password, through stdin or using a
// password file.
pub async fn run(email: &str, password_file: &Option<String>) -> Result<()> {
    storage::setup().await.context("Setup storage")?;

    let password = if let Some(password_file) = &password_file {
        // Read password from file.
        let password =
            std::fs::read_to_string(password_file).context("Failed to read password file")?;
        password.trim_end().to_string() // trim possible newlines from the end of the pw
    } else {
        // Prompt for password.
        let password = rpassword::prompt_password("New password: ")?;
        let confirm = rpassword::prompt_password("Confirm password: ")?;

        if password != confirm {
            return Err(anyhow!("Passwords do not match"));
        }

        password
    };

    let _ = user::set_password_by_email(email, &password)
        .await
        .with_context(|| format!("Failed to reset password for user: {}", email))?;

    Ok(())
}
