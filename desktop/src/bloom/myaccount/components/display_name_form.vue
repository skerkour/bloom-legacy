<template>
  <div>
    <v-row>
      <v-col cols="12" class="pb-0">
        <v-text-field
          label="Display name"
          v-model="newDisplayName"
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
        <v-btn color="primary" :disabled="!updatable" class="mx-3">Update</v-btn>

      </v-col>
    </v-row>

  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class DisplayNameForm extends Vue {
  // props
  @Prop({ type: String, default: '' }) displayName!: string;

  // data
  newDisplayName = '';
  error = '';

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
}
</script>


<style lang="scss" scoped>
</style>
