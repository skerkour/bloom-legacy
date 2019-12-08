import Vue, { VNode } from 'vue'; // eslint-disable-line

// we disable eslint, because otherwise Vue and VNode are marked as non used

declare global {
  namespace JSX {
    // tslint:disable no-empty-interface
    interface Element extends VNode {}
    // tslint:disable no-empty-interface
    interface ElementClass extends Vue {}
    interface IntrinsicElements {
      [elem: string]: any
    }
  }
}
