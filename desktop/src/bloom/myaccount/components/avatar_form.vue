<template>
  <div>
    <v-hover v-slot:default="{ hover }">
      <v-avatar class="blm-pointer" size="60" @click="openAvatarUpload">
        <v-img :src="avatar" >
            <v-expand-transition>
            <div
              v-if="hover"
              class="d-flex transition-fast-in-fast-out grey darken-4 avatar-reveal white--text"
              style="height: 100%;"
            >
              Update
            </div>
          </v-expand-transition>
        </v-img>
      </v-avatar>
    </v-hover>
    <v-progress-circular
      :size="60"
      color="primary"
      indeterminate
      v-if="isLoading"
    />

    <input type="file" class="avatar-upload" ref="avatarupload" v-on:change="uploadAvatar()"/>
  </div>
</template>


<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';

@Component
export default class AvatarForm extends Vue {
  // props
  @Prop({ type: String, default: '' }) avatar!: string;

  // data
  isLoading = false;

  // computed
  // lifecycle
  // watch
  // methods
  uploadAvatar() {
  }

  openAvatarUpload() {
    const upload = this.$refs.avatarupload as HTMLElement;
    upload.click();
  }
}
</script>


<style lang="scss" scoped>
.avatar-reveal {
  align-items: center;
  bottom: 0;
  justify-content: center;
  opacity: .5;
  position: absolute;
  width: 100%;
}

.avatar-upload {
  display: none;
}
</style>
