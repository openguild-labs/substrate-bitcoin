
<div class="content-row">
<div class="content-col">

{{#include ./template/README.md}}

</div>

<div class="content-col">

<div class="tab">
  <button class="maintab tablinks active" onclick="switchMainTab(event, 'Template')">Template</button>
  <button class="maintab tablinks" onclick="switchMainTab(event, 'Solution')">Solution</button>
  <button class="maintab tablinks" onclick="switchMainTab(event, 'Diff')">Diff</button>
</div>

<div id="Template" class="maintab tabcontent active">

<div class="tab">
<button class="subtab tablinks file-template file-modified active" onclick="switchSubTab(event, 'node/src/chain_spec.rs')" data-id="node/src/chain_spec.rs">node/src/chain_spec.rs</button>
<button class="subtab tablinks file-template file-modified" onclick="switchSubTab(event, 'runtime/src/utxo.rs')" data-id="runtime/src/utxo.rs">runtime/src/utxo.rs</button>
</div>
<div id="template/node/src/chain_spec.rs" class="subtab tabcontent active" data-id="node/src/chain_spec.rs">

```rust
{{#include ./template/node/src/chain_spec.rs}}
```

</div>

<div id="template/runtime/src/utxo.rs" class="subtab tabcontent" data-id="runtime/src/utxo.rs">

```rust
{{#include ./template/runtime/src/utxo.rs}}
```

</div>



</div>

<div id="Solution" class="maintab tabcontent">

<div class="tab">
<button class="subtab tablinks file-solution file-modified active" onclick="switchSubTab(event, 'node/src/chain_spec.rs')" data-id="node/src/chain_spec.rs">node/src/chain_spec.rs</button>
<button class="subtab tablinks file-solution file-modified" onclick="switchSubTab(event, 'runtime/src/utxo.rs')" data-id="runtime/src/utxo.rs">runtime/src/utxo.rs</button>
</div>
<div id="solution/node/src/chain_spec.rs" class="subtab tabcontent active" data-id="node/src/chain_spec.rs">

```rust
{{#include ./solution/node/src/chain_spec.rs}}
```

</div>

<div id="solution/runtime/src/utxo.rs" class="subtab tabcontent" data-id="runtime/src/utxo.rs">

```rust
{{#include ./solution/runtime/src/utxo.rs}}
```

</div>



</div>

<div id="Diff" class="maintab tabcontent">


<div class="tab">
	<button class="difftab tablinks active" onclick="switchDiff(event, 'template.diff')" data-id="template.diff">template.diff</button>
	<button class="difftab tablinks" onclick="switchDiff(event, 'solution.diff')" data-id="solution.diff">solution.diff</button>
</div>
<div id="template.diff" class="difftab tabcontent active" data-id="template.diff">

```diff
{{#include ./template/template.diff}}
```

</div>
<div id="solution.diff" class="difftab tabcontent" data-id="solution.diff">

```diff
{{#include ./solution/solution.diff}}
```

</div>

</div>

</div>
</div>
