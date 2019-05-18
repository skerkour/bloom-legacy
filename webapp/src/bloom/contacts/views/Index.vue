<template>
  <blm-contacts-contacts v-if="$store.state.session" />
  <blm-contacts-landing v-else />
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import Landing from '../components/Landing.vue';
import Contacts from '../components/Contacts.vue';


@Component({
  components: {
    'blm-contacts-contacts': Contacts,
    'blm-contacts-landing': Landing,
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
