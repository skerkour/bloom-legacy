<template>
  <blm-phaser-scans v-if="$store.state.session" />
  <blm-phaser-landing v-else />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Landing from '../components/Landing.vue';
import Scans from '../components/Scans.vue';


@Component({
  components: {
    'blm-phaser-landing': Landing,
    'blm-phaser-scans': Scans,
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
