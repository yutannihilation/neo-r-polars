test_that("$date_range(), $datetime_range()", {
  expect_snapshot(
    pl$date_range(1, as.Date("2000-01-01")),
    error = TRUE
  )
  expect_snapshot(
    pl$date_range(as.Date("2000-01-01"), TRUE),
    error = TRUE
  )
  expect_snapshot(
    pl$datetime_range(1, as.POSIXct("2000-01-01")),
    error = TRUE
  )
  expect_snapshot(
    pl$datetime_range(as.POSIXct("2000-01-01"), TRUE),
    error = TRUE
  )
})

test_that("$date_ranges(), $datetime_ranges()", {
  expect_snapshot(
    pl$date_ranges(1, as.Date("2000-01-01")),
    error = TRUE
  )
  expect_snapshot(
    pl$date_ranges(as.Date("2000-01-01"), TRUE),
    error = TRUE
  )
  expect_snapshot(
    pl$datetime_ranges(1, as.POSIXct("2000-01-01")),
    error = TRUE
  )
  expect_snapshot(
    pl$datetime_ranges(as.POSIXct("2000-01-01"), TRUE),
    error = TRUE
  )
})
