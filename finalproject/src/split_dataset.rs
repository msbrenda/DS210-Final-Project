use polars::prelude::*;
use smartcore::linalg::basic::matrix::DenseMatrix;
use smartcore::linalg::basic::arrays::Array2;
use smartcore::linalg::basic::arrays::MutArray;
use smartcore::model_selection::train_test_split;

pub fn split(wine_data: DataFrame) 
    -> Result<(DenseMatrix<f64>, DenseMatrix<f64>, Vec<i64>, Vec<i64>), Box<dyn std::error::Error>> { 

    // Selecting the variables for the training dataset
    let variables = wine_data.select(vec![
        "fixed_acidity",
        "volatile_acidity",
        "citric_acid",
        "residual_sugar",
        "chlorides",
        "free_sulfur_dioxide",
        "total_sulfur_dioxide",
        "density",
        "pH",
        "sulphates",
        "alcohol",
        "color",
    ]);

    // Set the target column for the training dataset
    let target = wine_data.column("quality").unwrap();

    // Creating a DenseMatrix to store the features data
    let new_matrix = create_matrix(&variables.unwrap());

    // Extracting the target data and converting it into a vector
    let new_vector = target.i64()?.to_vec();

    let mut y: Vec<i64> = Vec::new();
    for values in new_vector.iter(){
        let val = values.unwrap();
        y.push(val);
    }

    // Splitting the data into training and test sets using a 80:20 ratio
    let (x_train, x_test, y_train, y_test) = 
        train_test_split(&new_matrix.unwrap(), &y, 0.2, true, None);

    Ok((x_train, x_test, y_train, y_test))
}

fn create_matrix(wine_data: &DataFrame) -> Result<DenseMatrix<f64>, Box<dyn std::error::Error>>{
    
    // Get the number of rows and columns in the dataset
    let nrows = wine_data.height();
    let ncols = wine_data.width();

    // Converting the DataFrame to a 1D array of 64-bit floats
    let new_array = wine_data.to_ndarray::<Float64Type>(Default::default()).unwrap().clone();

    // Creating a DenseMatrix to store the dataset
    let mut new_matrix: DenseMatrix<f64>  = Array2::zeros(nrows, ncols);

    // Initializing row and column indices for iteration
    let mut column: u32 = 0;
    let mut row: u32 = 0;

    // Loop to fill the DenseMatrix with data from the 1D array
    for index in new_array.iter(){

        // Converting row and column indices to usize type for matrix indexing
        let matrix_row = usize::try_from(row).unwrap();
        let matrix_column = usize::try_from(column).unwrap();

        // Set the value in the matrix at the specified row and column
        new_matrix.set((matrix_row, matrix_column), *index);

        // Move to the next row or reset to the beginning of the row
        if matrix_column == ncols-1 {
            column = 0;
            row += 1 ;
        } else{
            column += 1;
        }
    }
    Ok(new_matrix)
}
