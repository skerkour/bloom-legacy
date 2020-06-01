<template>
  <v-select
    class="text-left"
    :items="items"
    item-text="label"
    item-value="value"
    label="Select"
    return-object
    single-line
    :value="selected"
    @change="onChanged"
    dense
    hide-details
  />
</template>


<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import { Mutations } from '@/store';

@Component
export default class GroupSelector extends Vue {
  // props
  // data
  // computed
  get items(): any[] {
    let res = [{ label: 'Me', value: null }];
    const grp = this.$store.state.groups.map((group: any) => ({ label: group.name, value: group }));
    res = res.concat(grp);
    return res;
  }

  get selected(): any {
    if (this.$store.state.selectedGroup) {
      const { selectedGroup } = this.$store.state;
      return { label: selectedGroup.name, value: selectedGroup };
    }
    return { label: 'Me', value: null };
  }

  // lifecycle
  // watch
  // methods
  onChanged(selected: any) {
    this.$store.commit(Mutations.SET_SELECTED_GROUP.toString(), selected.value);
  }
}
</script>


<style lang="scss" scoped>
.v-select {
  max-width: 50px;
}
</style>
