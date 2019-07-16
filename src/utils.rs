pub fn remove_duplicates(mut values: Vec<String>) -> Vec<String> {
  values.sort();
  values.dedup();
  values
}
