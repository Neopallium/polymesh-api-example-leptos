use leptos::prelude::*;

use polymesh_dart::{
    curve_tree::*, AccountAssetRegistrationProof, AccountKeys, AssetMintingProof, AssetState,
    ACCOUNT_TREE_HEIGHT, ACCOUNT_TREE_L, ACCOUNT_TREE_M,
};

pub fn performance_now() -> f64 {
    web_sys::window()
        .and_then(|w| w.performance())
        .map(|p| p.now())
        .unwrap_or(0.0)
}

pub fn gen_account_keys(name: &str) -> AccountKeys {
    web_sys::console::time_with_label(name);
    let mut rng = rand::rngs::OsRng;
    let start = performance_now();
    // Generate a new set of Dart account keys
    let keys =
            //AccountKeys::from_seed("dart-test-seed").expect("Failed to generate Dart account keys");
            AccountKeys::rand(&mut rng).expect("Failed to generate Dart account keys");
    let elapsed = performance_now() - start;
    web_sys::console::time_end_with_label(name);
    log::info!("Generated {name} in {} ms", elapsed);
    keys
}

pub fn init_dart() -> bool {
    web_sys::console::time_with_label("Dart init");

    web_sys::console::time_with_label("Dart init account tree parameters");
    get_account_curve_tree_parameters();
    web_sys::console::time_end_with_label("Dart init account tree parameters");
    web_sys::console::time_with_label("Dart init asset tree parameters");
    get_asset_curve_tree_parameters();
    web_sys::console::time_end_with_label("Dart init asset tree parameters");

    web_sys::console::time_end_with_label("Dart init");

    true
}

pub fn wasm_bench_dart() {
    web_sys::console::time_with_label("WASM Dart benchmark");

    let mut rng = rand::rngs::OsRng;
    web_sys::console::time_with_label("Asset tree setup");
    let mut asset_tree = AssetCurveTree::new().expect("Failed to create asset tree");
    web_sys::console::time_end_with_label("Asset tree setup");

    // The account curve tree.
    web_sys::console::time_with_label("Account tree setup");
    let mut account_tree =
        ProverCurveTree::<ACCOUNT_TREE_L, ACCOUNT_TREE_M, AccountTreeConfig>::new(
            ACCOUNT_TREE_HEIGHT,
        )
        .expect("Failed to create account tree");
    let account_params = account_tree.params().clone();
    web_sys::console::time_end_with_label("Account tree setup");

    let issuer_keys = gen_account_keys("Gen issuer keys");
    let mediator_keys = gen_account_keys("Gen mediator keys");
    let mediator_acct = mediator_keys.public_keys();
    let ctx = b"benchmark";

    let asset_id = 0 as _;
    let asset_state = AssetState::new(asset_id, &[mediator_acct.enc], &[]);

    // Create the asset.
    web_sys::console::time_with_label("Insert asset state into asset tree");
    asset_tree
        .set_asset_state(asset_state.clone())
        .expect("Failed to insert asset state into asset tree");
    let _asset_root = asset_tree.root().expect("Failed to get asset tree root");
    web_sys::console::time_end_with_label("Insert asset state into asset tree");

    // Generate a proof to benchmark verification.
    web_sys::console::time_with_label("Generate account asset registration proof");
    let (_proof, mut account_state) = AccountAssetRegistrationProof::new(
        &mut rng,
        &issuer_keys.acct,
        asset_id,
        0,
        ctx,
        &account_params,
    )
    .expect("Failed to generate proof");
    web_sys::console::time_end_with_label("Generate account asset registration proof");

    web_sys::console::time_with_label(
        "Update account tree with new leaf from asset registration proof",
    );
    account_state
        .commit_pending_state()
        .expect("Failed to commit pending state");
    let current_commitment = account_state
        .current_commitment(&issuer_keys.acct)
        .expect("Failed to get current commitment");
    let leaf = current_commitment
        .as_leaf_value()
        .expect("Failed to get leaf value from asset state commitment");
    account_tree
        .insert(leaf)
        .expect("Failed to insert asset state commitment into account tree");
    web_sys::console::time_end_with_label(
        "Update account tree with new leaf from asset registration proof",
    );

    // Benchmark: Generate asset minting proof.
    web_sys::console::time_with_label("Generate asset minting proof");
    let mint_amount = 1000u64;
    let proof = AssetMintingProof::new(
        &mut rng,
        &issuer_keys.acct,
        &mut account_state,
        &account_tree,
        mint_amount,
    )
    .expect("Failed to generate asset minting proof");
    web_sys::console::time_end_with_label("Generate asset minting proof");

    // Add the asset minting to the account state.
    web_sys::console::time_with_label("Update account tree with new leaf from asset minting proof");
    account_state
        .commit_pending_state()
        .expect("Failed to commit pending state");
    account_tree
        .insert(
            proof
                .updated_account_state_commitment
                .as_leaf_value()
                .expect("Failed to get leaf value from asset state commitment"),
        )
        .expect("Failed to insert updated account state commitment into account tree");
    let _account_root = (&account_tree)
        .root()
        .expect("Failed to get account tree root");
    web_sys::console::time_end_with_label(
        "Update account tree with new leaf from asset minting proof",
    );

    web_sys::console::time_end_with_label("WASM Dart benchmark");
}

#[component]
pub fn DartTest() -> impl IntoView {
    let dart_init = Memo::new(move |_| init_dart());
    let initialized = dart_init.get_untracked();
    log::info!("Dart initialized: {}", initialized);

    // Setup some account keys.
    let issuer_keys = Memo::new(move |_| gen_account_keys("Gen issuer keys"));

    let value = Memo::new(move |_| {
        // Generate a new set of Dart account keys
        let keys = issuer_keys.get();
        wasm_bench_dart();
        format!("Dart Account Keys:\nPublic Key: {:?}", keys.public_keys())
    });
    view! {
        <div class="tile is-ancestor is-vertical">
            <div class="tile is-child hero">
                <div class="hero-body container pb-0">
                    <p>{value}</p>
                </div>
            </div>
        </div>
    }
}
