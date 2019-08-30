# Vue


## Typescript component

```vue
<template>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';


@Component
export default class Component extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}

//@Prop({ type: String, default: 'right' }) x!: string;
//@Watch('pagination', { deep: true })

</script>


<style lang="scss" scoped>
</style>
```
