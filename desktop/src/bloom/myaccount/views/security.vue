<template>
  <v-container fluid class="text-left">

    <v-alert icon="mdi-alert-circle" type="error" :value="error !== ''">
      {{ error }}
    </v-alert>

    <v-row>
      <v-col cols="12">
        <div class="headline">
          Change password
        </div>
        <div>
          All other devices will be disconnected
        </div>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="9">
        <blm-myaccount-form-change-password />
      </v-col>
    </v-row>
  </v-container>
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import ChangePasswordForm from '../components/change_password_form.vue';

@Component({
  components: {
    'blm-myaccount-form-change-password': ChangePasswordForm,
  },
})
export default class Security extends Vue {
  // props
  // data
  error = '';

  // computed
  // lifecycle
  // watch
  // methods
}
</script>


<style lang="scss" scoped>
</style>
