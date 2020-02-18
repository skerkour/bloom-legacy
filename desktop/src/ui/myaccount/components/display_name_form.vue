<template>
  <div>
    <v-row>
      <v-col cols="12" class="pb-0">
        <v-text-field
          label="Display name"
          v-model="newDisplayName"
          hint="This is your public name, 'Sylvain kerkour' for example"
          persistent-hint
        />
      </v-col>
    </v-row>

    <v-row v-if="error !== ''">
      <v-col cols="12">
        <v-alert icon="mdi-alert-circle" type="error">
          {{ error }}
        </v-alert>
      </v-col>
    </v-row>

    <v-row>
      <v-col cols="12" class="text-right pt-0">
        <v-btn text :disabled="!updatable" @click="cancel">Cancel</v-btn>
        <v-btn color="primary" :disabled="!updatable" class="mx-3" @click="update"
          :loading="isLoading">Update</v-btn>

      </v-col>
    </v-row>

  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import * as models from '@/api/models';
import core from '@/core';
import { Method } from '@/core/users';

@Component
export default class DisplayNameForm extends Vue {
  // props
  @Prop({ type: String, default: '' }) displayName!: string;

  // data
  newDisplayName = '';
  error = '';
  isLoading = false;

  // computed
  get updatable() {
    return this.displayName !== this.newDisplayName;
  }

  // lifecycle
  created() {
    this.cancel();
  }

  // watch
  // methods
  cancel() {
    this.newDisplayName = this.displayName;
  }

  async update() {
    this.error = '';
    this.isLoading = true;

    const input: models.UserProfileInput = {
      displayName: this.newDisplayName,
    };
    try {
      const user: models.User = await core.call(Method.UpdateProfile, input);
      this.$emit('updated', user.displayName);
    } catch (err) {
      this.error = err.message;
    } finally {
      this.isLoading = false;
    }
  }
}
</script>


<style lang="scss" scoped>
</style>
