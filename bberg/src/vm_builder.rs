use ast::analyzed::Analyzed;

use itertools::Itertools;
use number::FieldElement;
use pil_analyzer::pil_analyzer::inline_intermediate_polynomials;

use crate::circuit_builder::CircuitBuilder;
use crate::composer_builder::ComposerBuilder;
use crate::file_writer::BBFiles;
use crate::flavor_builder::FlavorBuilder;
use crate::prover_builder::ProverBuilder;
use crate::relation_builder::{create_identities, create_row_type, RelationBuilder};
use crate::verifier_builder::VerifierBuilder;

// TODO: move to util
fn sanitize_name(string: &String) -> String {
    string.replace(".", "_").replace("[", "_").replace("]", "_")
}

pub(crate) fn analyzed_to_cpp<F: FieldElement>(
    analyzed: &Analyzed<F>,
    fixed: &[(String, Vec<F>)],
    witness: &[(String, Vec<F>)],
    name: Option<String>,
) -> BBFiles {
    let file_name: &str = &name.unwrap_or("Example".to_owned());

    let mut bb_files = BBFiles::default(file_name.to_owned());

    // Inlining step to remove the intermediate poly definitions
    let analyzed_identities = inline_intermediate_polynomials(analyzed);

    let (subrelations, identities, mut collected_shifts) = create_identities(&analyzed_identities);
    let shifted_polys: Vec<String> = collected_shifts.drain().collect_vec();

    // Collect all column names and determine if they need a shift or not
    let (
        fixed_names,
        witness_names,
        all_cols,
        unshifted,
        to_be_shifted,
        shifted,
        all_cols_with_shifts,
    ) = get_all_col_names(fixed, witness, &shifted_polys);

    let row_type = create_row_type(&all_cols_with_shifts);

    // ----------------------- Create the relation file -----------------------
    bb_files.create_relation_hpp(
        file_name,
        &subrelations,
        &identities,
        &row_type,
        &all_cols_with_shifts,
    );

    // ----------------------- Create the circuit builder file -----------------------
    bb_files.create_circuit_builder_hpp(file_name, &all_cols, &to_be_shifted);

    // ----------------------- Create the flavor file -----------------------
    bb_files.create_flavor_hpp(
        file_name,
        &subrelations,
        &fixed_names,
        &witness_names,
        &all_cols,
        &to_be_shifted,
        &shifted,
        // &shifted,
    );

    // ----------------------- Create the composer files -----------------------
    bb_files.create_composer_cpp(file_name, &all_cols);
    bb_files.create_composer_hpp(file_name);

    // ----------------------- Create the Verifier files -----------------------
    bb_files.create_verifier_cpp(file_name, &witness_names);
    bb_files.create_verifier_hpp(file_name);

    // ----------------------- Create the Prover files -----------------------
    bb_files.create_prover_cpp(file_name, &unshifted, &to_be_shifted);
    bb_files.create_prover_hpp(file_name);

    bb_files
}

fn get_all_col_names<F: FieldElement>(
    fixed: &[(String, Vec<F>)],
    witness: &[(String, Vec<F>)],
    to_be_shifted: &[String],
) -> (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<String>,
) {
    let fixed_names = fixed
        .iter()
        .map(|(name, _)| sanitize_name(name).to_owned())
        .collect::<Vec<_>>();
    let witness_names = witness
        .iter()
        .map(|(name, _)| sanitize_name(name).to_owned())
        .collect::<Vec<_>>();

    let shifted: Vec<String> = to_be_shifted
        .iter()
        .map(|name| format!("{}_shift", *name))
        .collect();

    let all_cols: Vec<String> = [fixed_names.clone(), witness_names.clone()]
        .into_iter()
        .flatten()
        .collect();

    let unshifted: Vec<String> = [fixed_names.clone(), witness_names.clone()]
        .into_iter()
        .flatten()
        .filter(|name| !shifted.contains(name))
        .collect();

    let with_shifts: Vec<String> = [fixed_names, witness_names, shifted.clone()]
        .into_iter()
        .flatten()
        .collect();

    (
        fixed_names,
        witness_names,
        all_cols,
        unshifted,
        to_be_shifted.to_vec(),
        shifted,
        with_shifts,
    )
}