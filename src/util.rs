use std::str::FromStr;

pub trait ParseAt {
    /// Attempts to parse a string to the specified type `F`, at the specified index `i` in a `Vec`
    ///
    /// # Examples
    /// ```
    /// let sample_input = "goto 1.0 1.0";
    /// let args = sample_input.split_whitespace().collect::<Vec<&str>>();
    ///
    /// let x_coord = args.parse_at::<f32>(1).unwrap();
    /// let y_coord = args.parse_at::<f32>(2).unwrap();
    /// ```
    fn parse_at<F: FromStr>(&self, i: usize) -> Result<F, F::Err>;
}

impl ParseAt for Vec<&str> {
    fn parse_at<F: FromStr>(&self, i: usize) -> Result<F, F::Err> {
        self.get(i).unwrap_or(&"").parse::<F>()
    }
}

impl ParseAt for Vec<String> {
    fn parse_at<F: FromStr>(&self, i: usize) -> Result<F, F::Err> {
        self.get(i).unwrap_or(&String::new()).parse::<F>()
    }
}