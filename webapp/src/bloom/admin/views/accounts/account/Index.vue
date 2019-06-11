<template>
  <blm-admin-account />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Landing from '../components/Landing.vue';
import Account from '../../../components/Account.vue';


@Component({
  components: {
    'blm-admin-account': Account,
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
