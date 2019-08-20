<template>
  <v-layout row wrap>

    <v-flex hidden-xs-only sm3>
      <v-subheader>Display Name</v-subheader>
    </v-flex>
    <v-flex xs12 sm9>
      <v-text-field
      v-model="display_name"
      label="Display name"
      :disabled="is_loading"
      counter="42"
      ></v-text-field>
    </v-flex>

    <v-flex xs12 text-xs-center v-if="error">
      <v-alert icon="mdi-alert-circle" :value="error" type="error">
        {{ error }}
      </v-alert>
    </v-flex>

    <v-flex class="text-xs-right" xs12>
      <v-btn flat @click="cancel" :disabled="is_loading || !updatable">Cancel</v-btn>
      <v-btn color="primary" @click="update_display_name" :loading="is_loading" :disabled="!updatable">
        Update
      </v-btn>
    </v-flex>

  </v-layout>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class DisplayNameForm extends Vue {
  // props
  @Prop({ type: String, default: '' }) displayname!: string;


  // data
  is_loading = false;
  display_name = '';
  error = '';


  // computed
  get updatable() {
    return this.displayname !== this.display_name;
  }


  // lifecycle
  created() {
    this.cancel();
  }


  // watch
  // methods
  async update_display_name() {
    try {
      this.is_loading = true;
      const payload = {
        display_name: this.display_name,
      };
      const res = await api.put(`${api.MYACCOUNT}/v1/me`, payload);
      this.$toast.success('Success', { icon: 'mdi-check-circle' });
      this.$emit('update', res);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.is_loading = false;
    }
  }

  cancel() {
    this.display_name = this.displayname;
  }
}
</script>
