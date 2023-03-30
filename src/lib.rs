mod content;

pub use content::extract_url_content;

#[cfg(test)]
mod tests {
    use crate::content::extract_url_content;

    #[tokio::test]
    async fn it_works() {
        let result = extract_url_content("https://levelup.gitconnected.com/are-programmers-getting-replaced-by-ai-gpt-4s-top-strategies-to-future-proof-your-coding-career-a5a2d4ba95ed")
            .await
            .unwrap();

        dbg!(result.clone().unwrap());
        assert_ne!(result, None);
    }
}
