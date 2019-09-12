<template>
  <v-container>
    <v-layout row wrap justify-left>
      <v-flex xs12 sm10 md8 xl7>
        <blm-myaccount-form-password />
      </v-flex>
    </v-layout>
  </v-container>
</template>


<script lang="ts">
import PasswordForm from '../components/PasswordForm.vue';
import { Component, Prop, Vue } from 'vue-property-decorator';


@Component({
  components: {
    'blm-myaccount-form-password': PasswordForm,
  },
})
export default class Devices extends Vue {
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
