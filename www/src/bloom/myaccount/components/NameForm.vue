<template>
  <v-layout row wrap>

    <v-flex hidden-xs-only sm3>
      <v-subheader>First name</v-subheader>
    </v-flex>
    <v-flex xs12 sm9>
      <v-text-field
      v-model="first_name"
      label="First name"
      :disabled="is_loading"
      ></v-text-field>
    </v-flex>

    <v-flex hidden-xs-only sm3>
      <v-subheader>Last name</v-subheader>
    </v-flex>
    <v-flex xs12 sm9>
      <v-text-field
      v-model="last_name"
      label="Last name"
      :disabled="is_loading"
      ></v-text-field>
    </v-flex>

    <v-flex xs12 text-xs-center v-if="error">
      <v-alert icon="mdi-alert-circle" :value="error" type="error">
        {{ error }}
      </v-alert>
    </v-flex>

    <v-flex class="text-xs-right" xs12>
      <v-btn flat @click="cancel" :disabled="is_loading || !updatable">Cancel</v-btn>
      <v-btn color="primary" @click="update_name" :loading="is_loading" :disabled="!updatable">
        Update
      </v-btn>
    </v-flex>

  </v-layout>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class NameForm extends Vue {
  // props
  @Prop({ type: String, default: '' }) firstname!: string;
  @Prop({ type: String, default: '' }) lastname!: string;


  // data
  is_loading = false;
  first_name = '';
  last_name = '';
  error = '';


  // computed
  get updatable() {
    return this.firstname !== this.first_name
      || this.lastname !== this.last_name;
  }


  // lifecycle
  created() {
    this.cancel();
  }


  // watch
  // methods
  async update_name() {
    try {
      this.is_loading = true;
      const payload = {
        first_name: this.first_name,
        last_name: this.last_name,
      };
      const res = await api.put(`${api.MYACCOUNT}/v1/me`, payload);
      this.$toast.success('Success', { icon: 'mdi-check-circle' });
      this.$emit('update', res);
      this.$store.commit('set_account', res);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  cancel() {
    this.first_name = this.firstname;
    this.last_name = this.lastname;
  }
}
</script>
