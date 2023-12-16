//main.rs

mod preprocessing;
mod graph;
mod split_dataset;
mod classification;

use polars::prelude::*;

fn main() {
    // Loading the wine dataset from a winequality.csv file 
    let file_path = r"/Users/brenda/Desktop/finalproject/winequality.csv";
    let wine_data = CsvReader::from_path(file_path)
        .unwrap()
        .has_header(true)
        .finish()
        .unwrap();

    // URLs for red and white wine datasets to create quality vs. alcohol graphs.
    let red_data = "http://archive.ics.uci.edu/ml/machine-learning-databases/wine-quality/winequality-red.csv";
    let white_data = "http://archive.ics.uci.edu/ml/machine-learning-databases/wine-quality/winequality-white.csv";
    
    // saving the generated graphs for red and white wine qualities.
    let red_wine_output_path = "red_wine_quality.png";
    let white_wine_output_path = "white_wine_quality.png";

    // Creating graphs to show the relationship between wine quality and alcohol for red and white wines.
    // red wine
    if let Err(err) = graph::create_graph(red_wine_output_path, red_data, "Red Wine Quality vs Alcohol") {
        eprintln!("Error creating red wine graph: {}", err);
    }
    // white wine
    if let Err(err) = graph::create_graph(white_wine_output_path, white_data, "White Wine Quality vs Alcohol") {
        eprintln!("Error creating white wine graph: {}", err);
    }

    // Cleaning the wine dataset to prepare it for analysis and printing the cleaned data.
    let wine_data_cleaned = preprocessing::cleaned_data(wine_data.clone()).unwrap();
    println!("{:?}", wine_data_cleaned);

    // Splitting the cleaned wine dataset into training and testing sets.
    let (x_train, x_test, y_train, y_test) = split_dataset::split(wine_data_cleaned.clone()).unwrap();

    // Using the k-nearest neighbors and decision tree algorithms for wine quality classification.
    classification::knn_classify(x_train.clone(), x_test.clone(), y_train.clone(), y_test.clone());
    classification::dt_classify(x_train.clone(), x_test.clone(), y_train.clone(), y_test.clone());

}

// Test whether the data is successfully loaded.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_loading() {
        // Loading the wine dataset and performing basic checks
        let file_path = r"/Users/brenda/Desktop/finalproject/winequality.csv";
        let wine_data = CsvReader::from_path(file_path)
            .unwrap()
            .has_header(true)
            .finish()
            .unwrap();

        assert_eq!(wine_data.height(), 6497);
        assert_eq!(wine_data.width(), 13);
    }

    #[test]
    fn test_graph_creation() {
        // Test graph creation functions for red and white wine datasets
        let red_data = "http://archive.ics.uci.edu/ml/machine-learning-databases/wine-quality/winequality-red.csv";
        let white_data = "http://archive.ics.uci.edu/ml/machine-learning-databases/wine-quality/winequality-white.csv";
        
        let red_wine_output_path = "red_wine_quality.png";
        let white_wine_output_path = "white_wine_quality.png";

        assert!(red_result.is_ok());
        assert!(white_result.is_ok());
    }
}
