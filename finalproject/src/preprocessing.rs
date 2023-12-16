// preprocessing.rs

use polars::prelude::*;

// Function for cleaning DataFrame
pub fn cleaned_data(mut wine_data: DataFrame) -> Result<DataFrame, Box<dyn std::error::Error>> {

    wine_data = wine_data
        .clone()
        .lazy()
        // Fill null values with 0.0 in columns
        .with_columns([col("fixed_acidity").fill_null(0.0), 
            col("volatile_acidity").fill_null(0.0), 
            col("citric_acid").fill_null(0.0), 
            col("residual_sugar").fill_null(0.0), 
            col("chlorides").fill_null(0.0), 
            col("sulphates").fill_null(0.0), 
            col("pH").fill_null(0.0),])
        .collect()?; // Collect the DataFrame after filling nulls

    // Check and display the count of missing values after the cleaning process 
    let null_count = wine_data
        .get_columns()
        .iter()
        .map(|col| col.is_null().sum())
        .collect::<Vec<_>>();

    println!("Check missing values after cleaning: {:?}", null_count);
    
    // Function to replace color names red and white with numeric values 1 and 2
    fn replace_color(wine_data: &mut DataFrame, column_name: &str, pattern: &str, replacep: &str) {
        let color = wine_data.column(column_name).unwrap();
        let numeric = color
            .utf8()
            .unwrap()
            .apply(|s| s.map(|v| v.replace(pattern, replacep).into()))
            .into_series();
        let _ = wine_data.replace(column_name, numeric);
    }

    replace_color(&mut wine_data, "color", "red", "1");
    replace_color(&mut wine_data, "color", "white", "2");

    let out = wine_data
    .clone()
    .lazy()
    .select([
        col("color").cast(DataType::Int64)
    ])
    .collect()?;

    let _ = wine_data.replace("color", (*out.column("color").unwrap()).clone());

    Ok(wine_data)

}