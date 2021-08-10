import React from 'react';
import LinearProgress, {
  LinearProgressProps,
} from '@material-ui/core/LinearProgress';
import Typography from '@material-ui/core/Typography';
import Box from '@material-ui/core/Box';

export const LinearProgressWithLabel = (
  props: LinearProgressProps & { value: number }
) => (
  <Box display="flex" alignItems="center">
    <Box width="100%" mr={1}>
      <LinearProgress variant="determinate" {...props} />
    </Box>
    <Box minWidth={35}>
      <Typography variant="body2" color="textSecondary">
        {Math.round(props.value)}&nbsp;%
      </Typography>
    </Box>
  </Box>
);
