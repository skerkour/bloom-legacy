<template>
  <blm-gallery-album />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Album from '../../components/Album.vue';


@Component({
  components: {
    'blm-gallery-album': Album,
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
