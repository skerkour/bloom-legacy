<template>
  <blm-gallery-gallery v-if="$store.state.session" />
  <blm-gallery-landing v-else />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Landing from '../components/Landing.vue';
import Gallery from '../components/Gallery.vue';


@Component({
  components: {
    'blm-gallery-gallery': Gallery,
    'blm-gallery-landing': Landing,
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
