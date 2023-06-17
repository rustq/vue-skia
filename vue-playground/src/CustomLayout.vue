<template>
  <div class="preview-code">
    <div>
      <slot name="editor"></slot>
    </div>
    <div>
      <slot name="preview"></slot>
    </div>
  </div>
</template>
<style>
.preview-code {
  display: flex;
  width: 950px;
  margin: 30px auto;
  border-radius: 10px;
  border: 1px solid #ccc;
  box-shadow: 0 0 3px #ccc;
}

.preview-code > div {
  width: 100%;
}

.preview-code > div:first-child {
  border-radius: 10px 0 0 10px;
  overflow: hidden;
}

.preview-code > div:last-child {
  padding: 30px;
  background-color: #fff;
  text-align: center;
  border-radius: 0 10px 10px 0;
}

@media only screen and (max-width: 1000px) {
  .preview-code {
    width: 90vw;
  }
}

@media only screen and (max-width: 568px) {
  .preview-code {
    display: block;
  }

  .preview-code > div {
    width: auto;
  }

  .preview-code > div:first-child {
    border-radius: 10px 10px 0 0;
  }

  .preview-code > div:last-child {
    border-radius: 0 0 10px 10px;
  }
}
</style>
