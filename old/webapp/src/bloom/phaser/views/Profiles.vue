<template>
  <blm-phaser-profiles />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Profiles from '../components/Profiles.vue';


@Component({
  components: {
    'blm-phaser-profiles': Profiles,
  },
})
export default class ProfilesView extends Vue {
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
