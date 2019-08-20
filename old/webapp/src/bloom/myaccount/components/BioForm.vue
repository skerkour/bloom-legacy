<template>
  <v-layout row wrap>

    <v-flex hidden-xs-only sm3>
      <v-subheader>Bio</v-subheader>
    </v-flex>
    <v-flex xs12 sm9>
      <v-textarea
      v-model="bio_data"
      label="Bio"
      counter="200"
      :disabled="is_loading"
      ></v-textarea>
    </v-flex>

    <v-flex xs12 text-xs-center v-if="error">
      <v-alert icon="mdi-alert-circle" :value="error" type="error">
        {{ error }}
      </v-alert>
    </v-flex>

    <v-flex class="text-xs-right" xs12>
      <v-btn flat @click="cancel" :disabled="is_loading || !updatable">Cancel</v-btn>
      <v-btn color="primary" @click="update_bio" :loading="is_loading" :disabled="!updatable">
        Update
      </v-btn>
    </v-flex>

  </v-layout>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import api from '@/bloom/kernel/api';


@Component
export default class BioForm extends Vue {
  // props
  @Prop({ type: String, default: '' }) bio!: string;


  // data
  is_loading = false;
  bio_data = '';
  error = '';


  // computed
  get updatable() {
    return this.bio !== this.bio_data;
  }


  // lifecycle
  created() {
    this.cancel();
  }


  // watch
  // methods
  async update_bio() {
    try {
      this.is_loading = true;
      const payload = {
        bio: this.bio_data,
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
    this.bio_data = this.bio;
  }
}
</script>
