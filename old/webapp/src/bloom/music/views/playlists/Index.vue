<template>
  <blm-music-playlists />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Playlists from '../../components/Playlists.vue';


@Component({
  components: {
    'blm-music-playlists': Playlists,
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
