<template>
  <blm-admin-accounts />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Landing from '../components/Landing.vue';
import Accounts from '../../components/Accounts.vue';


@Component({
  components: {
    'blm-admin-accounts': Accounts,
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
