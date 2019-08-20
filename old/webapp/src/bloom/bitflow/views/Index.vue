<template>
  <blm-bitflow-downloads v-if="$store.state.session" />
  <blm-bitflow-landing v-else/>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Landing from '../components/Landing.vue';
import Downloads from '../components/Downloads.vue';


@Component({
  components: {
    'blm-bitflow-downloads': Downloads,
    'blm-bitflow-landing': Landing,
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
