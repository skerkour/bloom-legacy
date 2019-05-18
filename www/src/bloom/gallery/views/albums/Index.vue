<template>
  <blm-gallery-albums />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Albums from '../../components/Albums.vue';


@Component({
  components: {
    'blm-gallery-albums': Albums,
  },
})
export default class Index extends Vue {
  // props
  // data
  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style scoped lang="scss">
</style>
