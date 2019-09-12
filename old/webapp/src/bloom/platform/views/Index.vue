<template>
  <blm-platform-dashboard v-if="$store.state.session" />
  <blm-platform-landing v-else />
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Dashboard from '../components/Dashboard.vue';
import Landing from '../components/Landing.vue';


@Component({
  components: {
    'blm-platform-dashboard': Dashboard,
    'blm-platform-landing': Landing,
  },
})
export default class Platform extends Vue {
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
