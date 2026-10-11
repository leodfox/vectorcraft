# Ardoise web editor (vectorcraft fork)

Ardoise Vecteur: web build of the Ardoise editor, based on VectorCraft by the ArtCraft Team (Apache-2.0 OR MIT).
Branch `ardoise` starts from upstream tag v0.9.0 (https://github.com/storytold/vectorcraft).

Changes in this fork: postMessage bridge for iframe embedding (apps/vectorcraft-web/src/web.rs),
Ardoise branding in the UI (the upstream name, wordmark and logos are not used), single web-only CI
workflow (.github/workflows/web.yml).

Build the web bundle: `rustup target add wasm32-unknown-unknown && cargo install trunk --locked && packaging/web/package.sh`
(the zip lands in dist/release/).

Licences: see LICENSE-APACHE, LICENSE-MIT and NOTICE. The copyright notices of the original authors are kept.
