import { newSpecPage } from '@stencil/core/testing';
import { WaitingRoom } from './waiting-room';

describe('waiting-room', () => {
  it('renders', async () => {
    const { root } = await newSpecPage({
      components: [WaitingRoom],
      html: '<waiting-room></waiting-room>',
    });
    expect(root).toEqualHtml(`
      <waiting-room>
        <mock:shadow-root>
          <div>
            Queue position 
          </div>
        </mock:shadow-root>
      </waiting-room>
    `);
  });

  it('renders with values', async () => {
    const { root } = await newSpecPage({
      components: [WaitingRoom],
      html: `<waiting-room position="12"></waiting-room>`,
    });
    expect(root).toEqualHtml(`
      <waiting-room position="12">
        <mock:shadow-root>
        <div>
          Queue position 12
        </div>
        </mock:shadow-root>
      </waiting-room>
    `);
  });
});
