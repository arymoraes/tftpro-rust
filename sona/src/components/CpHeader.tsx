import { Group, Text } from '@mantine/core';
import { useViewportSize } from '@mantine/hooks';
import type { NextPage } from 'next';
import { useState } from 'react';

const localStyles = {
  container: {
    backgroundColor: '#FF9500',
    display: 'flex',
    padding: '20px',
    width: '100%',
  },
};

const CpHeader: NextPage = () => {
  const [isOpen, setIsOpen] = useState(false);
  const { width } = useViewportSize();

  const toggleDrawer = () => {
    setIsOpen((prev) => !prev);
  };

  return (
    <nav>
      <Group sx={localStyles.container}>
        <Text size="lg" weight={500}>
          TFTPro
        </Text>
      </Group>
    </nav>
  );
};

export default CpHeader;
