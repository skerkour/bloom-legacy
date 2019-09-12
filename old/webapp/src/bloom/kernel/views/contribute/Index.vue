<template>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';


@Component
export default class Index extends Vue {
  // props
  // data
  hover_elevation = 4;


  // computed
  // lifecycle
  beforeCreate() {
    window.location.replace('https://gitlab.com/bloom42/docs/blob/master/contributing.md');
  }


  // watch
  // methods
}
</script>

<style scoped lang="scss">
a {
  text-decoration: none;
}

.font-18 {
  font-size: 18px;
}

.font-5 {
  font-size: 15px;
}
</style>
