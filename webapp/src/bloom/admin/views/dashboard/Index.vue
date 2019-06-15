<template>
  <blm-admin-dashboard />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Dashboard from '../../components/Dashboard.vue';


@Component({
  components: {
    'blm-admin-dashboard': Dashboard,
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
