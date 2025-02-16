
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
<button class="subtab tablinks file-source file-added active" onclick="switchSubTab(event, 'scripts/generate-signature/package-lock.json')" data-id="scripts/generate-signature/package-lock.json">scripts/generate-signature/package-lock.json</button>
<button class="subtab tablinks file-source file-added" onclick="switchSubTab(event, 'scripts/generate-signature/package.json')" data-id="scripts/generate-signature/package.json">scripts/generate-signature/package.json</button>
</div>
<div id="source/scripts/generate-signature/package-lock.json" class="subtab tabcontent active" data-id="scripts/generate-signature/package-lock.json">

```json
{{#include ./source/scripts/generate-signature/package-lock.json}}
```

</div>

<div id="source/scripts/generate-signature/package.json" class="subtab tabcontent" data-id="scripts/generate-signature/package.json">

```json
{{#include ./source/scripts/generate-signature/package.json}}
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
