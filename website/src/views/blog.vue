<template>
  <div></div>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';


@Component
export default class Index extends Vue {
  // props
  // data
  // computed
  // lifecycle
  created() {
    const route = this.$route.path.replace('/blog', '');
    window.location.href = `https://bloom42.gitlab.io/blog${route}`;
  }
  // watch
  // methods
}
</script>

<style scoped lang="scss">
</style>
