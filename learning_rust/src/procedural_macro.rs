#[cfg(test)]
mod tests {
    use ai_func_macro::function_to_string;

    const _OUTPUT: &str = "";

    #[function_to_string]
    fn function_for_ai_llm(_param: &str) {
        /// This will give the function to an LLM to produce and output in a structured form
        print!("{}",OUTPUT);
    }

    #[test]
    fn tests_proc_macro() {
        let x: &str = function_for_ai_llm("Message");
        dbg!(x);
    }
}