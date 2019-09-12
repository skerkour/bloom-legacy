<template>
  <blm-music-playlist />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Playlist from '../../components/Playlist.vue';


@Component({
  components: {
    'blm-music-playlist': Playlist,
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
