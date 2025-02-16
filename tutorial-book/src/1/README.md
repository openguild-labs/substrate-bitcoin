
<div class="content-row">
<div class="content-col">

{{#include ./source/README.md}}

</div>
<div class="content-col">

<div class="tab">
  <button class="maintab tablinks active" onclick="switchMainTab(event, 'Source')">Source</button>
  <button class="maintab tablinks" onclick="switchMainTab(event, 'Diff')">Diff</button>
</div>

<div id="Source" class="maintab tabcontent active">

<div class="tab">
<button class="subtab tablinks file-source file-added active" onclick="switchSubTab(event, 'Cargo.toml')" data-id="Cargo.toml">Cargo.toml</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'LICENSE')" data-id="LICENSE">LICENSE</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'multi-pow/Cargo.toml')" data-id="multi-pow/Cargo.toml">multi-pow/Cargo.toml</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'multi-pow/src/lib.rs')" data-id="multi-pow/src/lib.rs">multi-pow/src/lib.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'node/Cargo.toml')" data-id="node/Cargo.toml">node/Cargo.toml</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'node/build.rs')" data-id="node/build.rs">node/build.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'node/src/chain_spec.rs')" data-id="node/src/chain_spec.rs">node/src/chain_spec.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'node/src/cli.rs')" data-id="node/src/cli.rs">node/src/cli.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'node/src/command.rs')" data-id="node/src/command.rs">node/src/command.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'node/src/main.rs')" data-id="node/src/main.rs">node/src/main.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'node/src/rpc.rs')" data-id="node/src/rpc.rs">node/src/rpc.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'node/src/service.rs')" data-id="node/src/service.rs">node/src/service.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'runtime/Cargo.toml')" data-id="runtime/Cargo.toml">runtime/Cargo.toml</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'runtime/build.rs')" data-id="runtime/build.rs">runtime/build.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'runtime/src/block_author.rs')" data-id="runtime/src/block_author.rs">runtime/src/block_author.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'runtime/src/difficulty.rs')" data-id="runtime/src/difficulty.rs">runtime/src/difficulty.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'runtime/src/issuance.rs')" data-id="runtime/src/issuance.rs">runtime/src/issuance.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'runtime/src/lib.rs')" data-id="runtime/src/lib.rs">runtime/src/lib.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'runtime/src/utxo.rs')" data-id="runtime/src/utxo.rs">runtime/src/utxo.rs</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'rust-toolchain.toml')" data-id="rust-toolchain.toml">rust-toolchain.toml</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'rustfmt.toml')" data-id="rustfmt.toml">rustfmt.toml</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'spec.json')" data-id="spec.json">spec.json</button>
</div>
<div id="source/Cargo.toml" class="subtab tabcontent active" data-id="Cargo.toml">

```toml
{{#include ./source/Cargo.toml}}
```

</div>

<div id="source/LICENSE" class="subtab tabcontent" data-id="LICENSE">

```text
{{#include ./source/LICENSE}}
```

</div>

<div id="source/multi-pow/Cargo.toml" class="subtab tabcontent" data-id="multi-pow/Cargo.toml">

```toml
{{#include ./source/multi-pow/Cargo.toml}}
```

</div>

<div id="source/multi-pow/src/lib.rs" class="subtab tabcontent" data-id="multi-pow/src/lib.rs">

```rust
{{#include ./source/multi-pow/src/lib.rs}}
```

</div>

<div id="source/node/Cargo.toml" class="subtab tabcontent" data-id="node/Cargo.toml">

```toml
{{#include ./source/node/Cargo.toml}}
```

</div>

<div id="source/node/build.rs" class="subtab tabcontent" data-id="node/build.rs">

```rust
{{#include ./source/node/build.rs}}
```

</div>

<div id="source/node/src/chain_spec.rs" class="subtab tabcontent" data-id="node/src/chain_spec.rs">

```rust
{{#include ./source/node/src/chain_spec.rs}}
```

</div>

<div id="source/node/src/cli.rs" class="subtab tabcontent" data-id="node/src/cli.rs">

```rust
{{#include ./source/node/src/cli.rs}}
```

</div>

<div id="source/node/src/command.rs" class="subtab tabcontent" data-id="node/src/command.rs">

```rust
{{#include ./source/node/src/command.rs}}
```

</div>

<div id="source/node/src/main.rs" class="subtab tabcontent" data-id="node/src/main.rs">

```rust
{{#include ./source/node/src/main.rs}}
```

</div>

<div id="source/node/src/rpc.rs" class="subtab tabcontent" data-id="node/src/rpc.rs">

```rust
{{#include ./source/node/src/rpc.rs}}
```

</div>

<div id="source/node/src/service.rs" class="subtab tabcontent" data-id="node/src/service.rs">

```rust
{{#include ./source/node/src/service.rs}}
```

</div>

<div id="source/runtime/Cargo.toml" class="subtab tabcontent" data-id="runtime/Cargo.toml">

```toml
{{#include ./source/runtime/Cargo.toml}}
```

</div>

<div id="source/runtime/build.rs" class="subtab tabcontent" data-id="runtime/build.rs">

```rust
{{#include ./source/runtime/build.rs}}
```

</div>

<div id="source/runtime/src/block_author.rs" class="subtab tabcontent" data-id="runtime/src/block_author.rs">

```rust
{{#include ./source/runtime/src/block_author.rs}}
```

</div>

<div id="source/runtime/src/difficulty.rs" class="subtab tabcontent" data-id="runtime/src/difficulty.rs">

```rust
{{#include ./source/runtime/src/difficulty.rs}}
```

</div>

<div id="source/runtime/src/issuance.rs" class="subtab tabcontent" data-id="runtime/src/issuance.rs">

```rust
{{#include ./source/runtime/src/issuance.rs}}
```

</div>

<div id="source/runtime/src/lib.rs" class="subtab tabcontent" data-id="runtime/src/lib.rs">

```rust
{{#include ./source/runtime/src/lib.rs}}
```

</div>

<div id="source/runtime/src/utxo.rs" class="subtab tabcontent" data-id="runtime/src/utxo.rs">

```rust
{{#include ./source/runtime/src/utxo.rs}}
```

</div>

<div id="source/rust-toolchain.toml" class="subtab tabcontent" data-id="rust-toolchain.toml">

```toml
{{#include ./source/rust-toolchain.toml}}
```

</div>

<div id="source/rustfmt.toml" class="subtab tabcontent" data-id="rustfmt.toml">

```toml
{{#include ./source/rustfmt.toml}}
```

</div>

<div id="source/spec.json" class="subtab tabcontent" data-id="spec.json">

```json
{{#include ./source/spec.json}}
```

</div>



</div>

<div id="Diff" class="maintab tabcontent">


<div class="tab">
	<button class="difftab tablinks active" onclick="switchDiff(event, 'changes.diff')" data-id="changes.diff">changes.diff</button>
</div>
<div id="changes.diff" class="difftab tabcontent active" data-id="changes.diff">

```diff
{{#include ./source/changes.diff}}
```

</div>

</div>

</div>
</div>
