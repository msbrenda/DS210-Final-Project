// classification.rs

use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::tree::decision_tree_classifier::*;
use smartcore::neighbors::knn_classifier::*;
use smartcore::metrics::accuracy;

// Decision Tree Classification
pub fn dt_classify(x_train: DenseMatrix<f64>, x_test: DenseMatrix<f64>, y_train: Vec<i64>, y_test: Vec<i64>) {

    // Fitting the data into the Decision Tree classifier
    let tree = DecisionTreeClassifier::fit(&x_train, &y_train, Default::default()).unwrap();

    // Making predictions using the trained Decision Tree model
    let prediction = tree.predict(&x_test).unwrap(); 

    // Calculating accuracy for the Decision Tree model
    let acc = accuracy(&y_test, &prediction);
    println!("Decision Tree's accuracy score is: {:?}", acc);
}

// K-Nearest Neighbors (KNN) Classification
pub fn knn_classify(x_train: DenseMatrix<f64>, x_test: DenseMatrix<f64>, y_train: Vec<i64>, y_test: Vec<i64>) {

    // Fitting the data into the KNN classifier
    let knn = KNNClassifier::fit(&x_train, &y_train, Default::default()).unwrap();

    // Making predictions using the trained KNN model
    let prediction = knn.predict(&x_test).unwrap();

    // Calculating accuracy for the KNN model
    let acc = accuracy(&y_test, &prediction);
    println!("KNN's accuracy is: {:?}", acc);

}