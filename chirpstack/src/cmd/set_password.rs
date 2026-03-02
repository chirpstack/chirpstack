use anyhow::{Context, Result};

use crate::storage::{self, user};

// This command sets an user password, either by prompting the password, through stdin or using a
// password file.
pub async fn run(email: &str, password_file: &Option<String>, stdin: bool) -> Result<()> {
    storage::setup().await.context("Setup storage")?;

    let password = get_password(password_file, stdin)?;

    let _ = user::set_password_by_email(email, &password)
        .await
        .with_context(|| format!("Failed to reset password for user: {}", email))?;

    Ok(())
}

fn get_password(password_file: &Option<String>, stdin: bool) -> Result<String> {
    if stdin {
        rpassword::read_password_from_bufread(&mut std::io::stdin().lock())
            .context("Failed to read password from stdin")
    } else if let Some(password_file) = &password_file {
        // Read password from file.
        let password =
            std::fs::read_to_string(password_file).context("Failed to read password file")?;
        Ok(password.trim_end().to_string()) // trim possible newlines from the end of the pw
    } else {
        // Prompt for password.
        let password = rpassword::prompt_password("New password: ")?;
        let confirm = rpassword::prompt_password("Confirm password: ")?;

        if password != confirm {
            return Err(anyhow!("Passwords do not match"));
        }

        Ok(password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn write_temp_file(contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "chirpstack_test_pw_{}.txt",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
        path
    }

    #[test]
    fn test_get_password_from_file() {
        let path = write_temp_file("secretpassword\n");
        let pw = get_password(&Some(path.to_str().unwrap().to_string()), false).unwrap();
        fs::remove_file(&path).ok();
        assert_eq!(pw, "secretpassword");
    }

    #[test]
    fn test_get_password_from_file_no_trailing_newline() {
        let path = write_temp_file("secretpassword");
        let pw = get_password(&Some(path.to_str().unwrap().to_string()), false).unwrap();
        fs::remove_file(&path).ok();
        assert_eq!(pw, "secretpassword");
    }

    #[test]
    fn test_get_password_from_file_crlf() {
        let path = write_temp_file("secretpassword\r\n");
        let pw = get_password(&Some(path.to_str().unwrap().to_string()), false).unwrap();
        fs::remove_file(&path).ok();
        assert_eq!(pw, "secretpassword");
    }

    #[test]
    fn test_get_password_file_not_found() {
        let result = get_password(
            &Some("/nonexistent/path/to/password.txt".to_string()),
            false,
        );
        assert!(result.is_err());
    }
}
