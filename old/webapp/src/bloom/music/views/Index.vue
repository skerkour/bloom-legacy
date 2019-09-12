<template>
  <blm-music-songs v-if="$store.state.session" />
  <blm-music-landing v-else />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Landing from '../components/Landing.vue';
import Songs from '../components/Songs.vue';


@Component({
  components: {
    'blm-music-landing': Landing,
    'blm-music-songs': Songs,
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
