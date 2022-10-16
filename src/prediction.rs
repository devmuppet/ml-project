pub(crate) use linfa_trees::DecisionTree;
pub(crate) use linfa::prelude::*;

fn predictions() {
    let (train, test) = linfa_datasets::iris()
        .split_with_ratio(0.9);

    let model = DecisionTree::params()
        .fit(&train).unwrap();
    
    let predictions = model.predict(&test);
    
    println!("{:?}", predictions);
    println!("{:?}", test.targets);
}