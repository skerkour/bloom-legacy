<template>
  <blm-public-profile />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import PublicProfile from '../components/PublicProfile.vue';

@Component({
  components: {
    'blm-public-profile': PublicProfile,
  },
})
export default class UserProfile extends Vue {
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
