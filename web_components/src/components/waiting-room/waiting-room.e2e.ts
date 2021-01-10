import { newE2EPage } from '@stencil/core/testing';

describe('waiting-room', () => {
  it('renders', async () => {
    const page = await newE2EPage();

    await page.setContent('<waiting-room></waiting-room>');
    const element = await page.find('waiting-room');
    expect(element).toHaveClass('hydrated');
  });

  it('renders changes to the name data', async () => {
    const page = await newE2EPage();

    await page.setContent('<waiting-room></waiting-room>');
    const component = await page.find('waiting-room');
    const element = await page.find('waiting-room >>> div');
    expect(element.textContent).toEqual(`Queue position `);

    component.setProperty('position', '12');
    await page.waitForChanges();
    expect(element.textContent).toEqual(`Queue position 12`);

    component.setProperty('position', '100');
    await page.waitForChanges();
    expect(element.textContent).toEqual(`Queue position 100`);

    component.setProperty('position', '123456');
    await page.waitForChanges();
    expect(element.textContent).toEqual(`Queue position 123456`);
  });
});
