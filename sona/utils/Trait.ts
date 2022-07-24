export const traitColorMapping = (tier: number) => {
  switch (tier) {
    case 1:
      return 'brown';
    case 2:
      return 'grey';
    case 3:
      return 'gold';
    case 4:
      return 'fff';
    case 5:
      return '#800000';
    default:
      return '#000000';
  }
}