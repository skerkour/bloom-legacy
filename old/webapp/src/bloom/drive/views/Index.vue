<template>
  <blm-drive-folder v-if="$store.state.session" />
  <blm-drive-landing v-else />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Landing from '../components/Landing.vue';
import Folder from '../components/Folder.vue';


@Component({
  components: {
    'blm-drive-folder': Folder,
    'blm-drive-landing': Landing,
  },
})
export default class IndexView extends Vue {
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
