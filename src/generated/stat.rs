use crate::*;
#[allow(unused_imports)]
use crate::stat::*;

///  Calculates the average of the absolute deviations of the values in list.
#[inline]
pub fn avedev<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVEDEV", n)
}

///  Average the set of numbers
#[inline]
pub fn average<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVERAGE", n)
}

///  Average values, including values of type Text and Logical
#[inline]
pub fn averagea<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("AVERAGEA", n)
}

///  Average the values of cells in a range that meet a criteria.
#[inline]
pub fn averageif<A: Reference, B: Criterion, C: Reference>(r: A, c: B, a: C) -> FnNumber3<A, B, C> {
    FnNumber3("AVERAGEIF", r, c, a)
}

///  returns the value of the probability density function or the cumulative distribution function for the beta distribution.
#[inline]
pub fn betadist___<A: Number, B: Number, C: Number, D: Number, E: Number, F: Logical>(x: A, alpha: B, beta: C, a: D, b: E, cumulative: F) -> FnNumber6<A, B, C, D, E, F> {
    FnNumber6("BETADIST", x, alpha, beta, a, b, cumulative)
}

///  returns the value of the probability density function or the cumulative distribution function for the beta distribution.
#[inline]
pub fn betadist__<A: Number, B: Number, C: Number, D: Number, E: Number>(x: A, alpha: B, beta: C, a: D, b: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("BETADIST", x, alpha, beta, a, b)
}

///  returns the value of the probability density function or the cumulative distribution function for the beta distribution.
#[inline]
pub fn betadist_<A: Number, B: Number, C: Number, D: Number>(x: A, alpha: B, beta: C, a: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BETADIST", x, alpha, beta, a)
}

///  returns the value of the probability density function or the cumulative distribution function for the beta distribution.
#[inline]
pub fn betadist<A: Number, B: Number, C: Number>(x: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("BETADIST", x, alpha, beta)
}

///  returns the inverse of BETADIST(x;α;β;A;B;TRUE())
#[inline]
pub fn betainv__<A: Number, B: Number, C: Number, D: Number, E: Number>(p: A, alpha: B, beta: C, a: D, b: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("BETAINV", p, alpha, beta, a, b)
}

///  returns the inverse of BETADIST(x;α;β;A;B;TRUE())
#[inline]
pub fn betainv_<A: Number, B: Number, C: Number, D: Number>(p: A, alpha: B, beta: C, a: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BETAINV", p, alpha, beta, a)
}

///  returns the inverse of BETADIST(x;α;β;A;B;TRUE())
#[inline]
pub fn betainv<A: Number, B: Number, C: Number>(p: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("BETAINV", p, alpha, beta)
}

///  Returns the probability of a trial result using binomial distribution.
#[inline]
pub fn binom_dist_range_<A: Number, B: Number, C: Number, D: Number>(n: A, p: B, s: C, s2: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BINOM.DIST.RANGE", n, p, s, s2)
}

///  Returns the probability of a trial result using binomial distribution.
#[inline]
pub fn binom_dist_range<A: Number, B: Number, C: Number>(n: A, p: B, s: C) -> FnNumber3<A, B, C> {
    FnNumber3("BINOM.DIST.RANGE", n, p, s)
}

///  
#[inline]
pub fn binomdist<A: Number, B: Number, C: Number, D: Logical>(s: A, n: B, p: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("BINOMDIST", s, n, p, cumulative)
}

///  returns the value of the probability density function or the cumulative distribution function for the χ2-distribution.
#[inline]
pub fn chisqdist_<A: Number, B: Number, C: Logical>(x: A, degrees_of_freedom: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("CHISQDIST", x, degrees_of_freedom, cumulative)
}

///  returns the value of the probability density function or the cumulative distribution function for the χ2-distribution.
#[inline]
pub fn chisqdist<A: Number, B: Number>(x: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("CHISQDIST", x, degrees_of_freedom)
}

///  Returns the confidence interval for a population mean
#[inline]
pub fn confidence<A: Number, B: Number, C: Number>(alpha: A, stddev: B, size: C) -> FnNumber3<A, B, C> {
    FnNumber3("CONFIDENCE", alpha, stddev, size)
}

///  Calculates the correlation coefficient of values in N1 and N2.
#[inline]
pub fn correl<A: Array, B: Array>(n1: A, n2: B) -> FnNumber2<A, B> {
    FnNumber2("CORREL", n1, n2)
}

///  Calculates covariance of two cell ranges.
#[inline]
pub fn covar<A: Array, B: Array>(n1: A, n2: B) -> FnNumber2<A, B> {
    FnNumber2("COVAR", n1, n2)
}

///  Returns the smallest value for which the cumulative binomial distribution is greater than or equal to a criterion value.
#[inline]
pub fn critbinom<A: Number, B: Number, C: Number>(trials: A, sp: B, alpha: C) -> FnNumber3<A, B, C> {
    FnNumber3("CRITBINOM", trials, sp, alpha)
}

///  Calculates sum of squares of deviations.
#[inline]
pub fn devsq<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("DEVSQ", n)
}

///  returns the value of the probability density function or the cumulative distribution function for the exponential distribution.
#[inline]
pub fn expondist_<A: Number, B: Number, C: Logical>(x: A, lambda: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("EXPONDIST", x, lambda, cumulative)
}

///  returns the value of the probability density function or the cumulative distribution function for the exponential distribution.
#[inline]
pub fn expondist<A: Number, B: Number>(x: A, lambda: B) -> FnNumber2<A, B> {
    FnNumber2("EXPONDIST", x, lambda)
}

///  returns the value of the probability density function or the cumulative distribution function for the F-distribution.
#[inline]
pub fn fdist_<A: Number, B: Number, C: Number, D: Logical>(x: A, r1: B, r2: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("FDIST", x, r1, r2, cumulative)
}

///  returns the value of the probability density function or the cumulative distribution function for the F-distribution.
#[inline]
pub fn fdist<A: Number, B: Number, C: Number>(x: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("FDIST", x, r1, r2)
}

///  returns the inverse of FDIST(x;R1;R2;TRUE()).
#[inline]
pub fn finv<A: Number, B: Number, C: Number>(p: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("FINV", p, r1, r2)
}

///  returns the Fisher transformation.
#[inline]
pub fn fisher<A: Number>(r: A) -> FnNumber1<A> {
    FnNumber1("FISHER", r)
}

///  returns the inverse Fisher transformation
#[inline]
pub fn fisherinv<A: Number>(r: A) -> FnNumber1<A> {
    FnNumber1("FISHERINV", r)
}

///  Extrapolates future values based on existing x and y values.
#[inline]
pub fn forecast<A: Number, B: Array, C: Array>(value: A, data_y: B, data_x: C) -> FnNumber3<A, B, C> {
    FnNumber3("FORECAST", value, data_y, data_x)
}

///  Categorizes values into intervals and counts the number of values in each interval
#[inline]
pub fn frequency<A: Sequence, B: Sequence>(data_source: A, bins: B) -> FnNumber2<A, B> {
    FnNumber2("FREQUENCY", data_source, bins)
}

///  Calculates the probability of an F-test
#[inline]
pub fn ftest<A: Sequence, B: Sequence>(data1: A, data2: B) -> FnNumber2<A, B> {
    FnNumber2("FTEST", data1, data2)
}

///  returns the value of the probability density function or the cumulative distribution function for the Gamma distribution.
#[inline]
pub fn gammadist_<A: Number, B: Number, C: Number, D: Logical>(x: A, alpha: B, beta: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("GAMMADIST", x, alpha, beta, cumulative)
}

///  returns the value of the probability density function or the cumulative distribution function for the Gamma distribution.
#[inline]
pub fn gammadist<A: Number, B: Number, C: Number>(x: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("GAMMADIST", x, alpha, beta)
}

///  returns the inverse of GAMMADIST(X;α;β;TRUE).
#[inline]
pub fn gammainv<A: Number, B: Number, C: Number>(p: A, alpha: B, beta: C) -> FnNumber3<A, B, C> {
    FnNumber3("GAMMAINV", p, alpha, beta)
}

///  Returns 0.5 less than the standard normal cumulative distribution
#[inline]
pub fn gauss<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("GAUSS", x)
}

///  returns the geometric mean of a sequence
#[inline]
pub fn geomean<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("GEOMEAN", n)
}

///  Returns predicted values based on an exponential regression.
#[inline]
pub fn growth_<A: Array, B: Array, C: Array, D: Logical>(known_y: A, known_x: Option<B>, new_x: Option<C>, const_: D) -> FnArray4<A, Option<B>, Option<C>, D> {
    FnArray4("GROWTH", known_y, known_x, new_x, const_)
}

///  Returns predicted values based on an exponential regression.
#[inline]
pub fn growth<A: Array, B: Array, C: Array>(known_y: A, known_x: Option<B>, new_x: Option<C>) -> FnArray3<A, Option<B>, Option<C>> {
    FnArray3("GROWTH", known_y, known_x, new_x)
}

///  returns the harmonic mean of a sequence
#[inline]
pub fn harmean<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("HARMEAN", n)
}

///  The hypergeometric distribution returns the number of successes in a sequence of n draws from a finite population without replacement.
#[inline]
pub fn hypgeomdist_<A: Number, B: Number, C: Number, D: Number, E: Logical>(x: A, t: B, m: C, n : D, cumulative: E) -> FnNumber5<A, B, C, D, E> {
    FnNumber5("HYPGEOMDIST", x, t, m, n , cumulative)
}

///  The hypergeometric distribution returns the number of successes in a sequence of n draws from a finite population without replacement.
#[inline]
pub fn hypgeomdist<A: Number, B: Number, C: Number, D: Number>(x: A, t: B, m: C, n : D) -> FnNumber4<A, B, C, D> {
    FnNumber4("HYPGEOMDIST", x, t, m, n )
}

///  Returns the y-intercept of the linear regression line for the given data.
#[inline]
pub fn intercept<A: Array, B: Array>(data_y: A, data_x: B) -> FnNumber2<A, B> {
    FnNumber2("INTERCEPT", data_y, data_x)
}

///  Return the kurtosis (“peakedness”) of a data set
#[inline]
pub fn kurt<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("KURT", x)
}

///  Finds the nth largest value in a list
#[inline]
pub fn large<A: Sequence, B: Number>(list: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("LARGE", list, n)
}

///  returns the right-tail probability for the χ2-distribution
#[inline]
pub fn legacy_chidist<A: Number, B: Number>(x: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("LEGACY.CHIDIST", x, degrees_of_freedom)
}

///  Returns some Chi square goodness-for-fit test.
#[inline]
pub fn legacy_chitest<A: Array, B: Array>(a: A, end_date: B) -> FnNumber2<A, B> {
    FnNumber2("LEGACY.CHITEST", a, end_date)
}

///  returns the area of the right tail of the probability density function for the F-distribution
#[inline]
pub fn legacy_fdist<A: Number, B: Number, C: Number>(x: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.FDIST", x, r1, r2)
}

///  returns the inverse of LEGACY.FDIST(x;R1;R2).
#[inline]
pub fn legacy_finv<A: Number, B: Number, C: Number>(p: A, r1: B, r2: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.FINV", p, r1, r2)
}

///  returns the value of the cumulative distribution function for the standard normal distribution.
#[inline]
pub fn legacy_normsdist<A: Number>(x: A) -> FnNumber1<A> {
    FnNumber1("LEGACY.NORMSDIST", x)
}

///  returns the inverse of LEGACY.NORMSDIST(X
#[inline]
pub fn legacy_normsinv<A: Number>(p: A) -> FnNumber1<A> {
    FnNumber1("LEGACY.NORMSINV", p)
}

///  Returns the area to the tail or tails of the probability density function of the tdistribution
#[inline]
pub fn legacy_tdist<A: Number, B: Number, C: Number>(x: A, df: B, tails: C) -> FnNumber3<A, B, C> {
    FnNumber3("LEGACY.TDIST", x, df, tails)
}

///  Returns the parameters of the (simple or multiple) linear regression equation for the given data and, optionally, statistics on this regression.
#[inline]
pub fn linest__<A: Array, B: Array, C: Logical, D: Logical>(known_y: A, known_x: B, const_: C, stats: D) -> FnArray4<A, B, C, D> {
    FnArray4("LINEST", known_y, known_x, const_, stats)
}

///  Returns the parameters of the (simple or multiple) linear regression equation for the given data and, optionally, statistics on this regression.
#[inline]
pub fn linest_<A: Array, B: Array, C: Logical>(known_y: A, known_x: B, const_: C) -> FnArray3<A, B, C> {
    FnArray3("LINEST", known_y, known_x, const_)
}

///  Returns the parameters of the (simple or multiple) linear regression equation for the given data and, optionally, statistics on this regression.
#[inline]
pub fn linest<A: Array, B: Array>(known_y: A, known_x: B) -> FnArray2<A, B> {
    FnArray2("LINEST", known_y, known_x)
}

///  Returns the parameters of an exponential regression equation for the given data obtained by linearizing this intrinsically linear response function and returns, optionally, statistics on this regression.
#[inline]
pub fn logest__<A: Array, B: Array, C: Logical, D: Logical>(known_y: A, known_x: Option<B>, const_: C, stats: D) -> FnArray4<A, Option<B>, C, D> {
    FnArray4("LOGEST", known_y, known_x, const_, stats)
}

///  Returns the parameters of an exponential regression equation for the given data obtained by linearizing this intrinsically linear response function and returns, optionally, statistics on this regression.
#[inline]
pub fn logest_<A: Array, B: Array, C: Logical>(known_y: A, known_x: Option<B>, const_: C) -> FnArray3<A, Option<B>, C> {
    FnArray3("LOGEST", known_y, known_x, const_)
}

///  Returns the parameters of an exponential regression equation for the given data obtained by linearizing this intrinsically linear response function and returns, optionally, statistics on this regression.
#[inline]
pub fn logest<A: Array, B: Array>(known_y: A, known_x: Option<B>) -> FnArray2<A, Option<B>> {
    FnArray2("LOGEST", known_y, known_x)
}

///  returns the inverse of LOGNORMDIST(x;Mean;StandardDeviation,TRUE()).
#[inline]
pub fn loginv<A: Number, B: Number, C: Number>(p: A, mean: Option<B>, std_dev: Option<C>) -> FnNumber3<A, Option<B>, Option<C>> {
    FnNumber3("LOGINV", p, mean, std_dev)
}

///  returns the value of the probability density function or the cumulative distribution function for the lognormal distribution with the mean and standard deviation given.
#[inline]
pub fn lognormdist_<A: Number, B: Number, C: Number, D: Logical>(x: A, my: Option<B>, sigma: Option<C>, cumulative: D) -> FnNumber4<A, Option<B>, Option<C>, D> {
    FnNumber4("LOGNORMDIST", x, my, sigma, cumulative)
}

///  returns the value of the probability density function or the cumulative distribution function for the lognormal distribution with the mean and standard deviation given.
#[inline]
pub fn lognormdist<A: Number, B: Number, C: Number>(x: A, my: Option<B>, sigma: Option<C>) -> FnNumber3<A, Option<B>, Option<C>> {
    FnNumber3("LOGNORMDIST", x, my, sigma)
}

/// Return the maximum from a set of numbers.
#[inline]
pub fn max<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MAX", n)
}

///  Return the maximum from a set of values, including values of type Text and Logical.
#[inline]
pub fn maxa<A: Any>(n: A) -> FnNumber1<A> {
    FnNumber1("MAXA", n)
}

///  Returns the median (middle) value in the list.
#[inline]
pub fn median<A: Sequence>(x: A) -> FnNumber1<A> {
    FnNumber1("MEDIAN", x)
}

/// Return the minimum from a set of numbers
#[inline]
pub fn min<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MIN", n)
}

///  Return the minimum from a set of values, including values of type Text and Logical
#[inline]
pub fn mina<A: Any>(n: A) -> FnNumber1<A> {
    FnNumber1("MINA", n)
}

///  Returns the most common value in a data se
#[inline]
pub fn mode<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("MODE", n)
}

///  Returns the negative binomial distribution.
#[inline]
pub fn negbinomdist<A: Number, B: Number, C: Number>(x: A, r: B, prob: C) -> FnNumber3<A, B, C> {
    FnNumber3("NEGBINOMDIST", x, r, prob)
}

///  returns the value of the probability density function or the cumulative distribution function for the normal distribution with the mean and standard deviation given.
#[inline]
pub fn normdist_<A: Number, B: Number, C: Number, D: Logical>(x: A, mean: B, std_dev: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("NORMDIST", x, mean, std_dev, cumulative)
}

///  returns the value of the probability density function or the cumulative distribution function for the normal distribution with the mean and standard deviation given.
#[inline]
pub fn normdist<A: Number, B: Number, C: Number>(x: A, mean: B, std_dev: C) -> FnNumber3<A, B, C> {
    FnNumber3("NORMDIST", x, mean, std_dev)
}

///  returns the inverse of NORMDIST(x;Mean;StandardDeviation,TRUE()).
#[inline]
pub fn norminv<A: Number, B: Number, C: Number>(p: A, mean: B, std_dev: C) -> FnNumber3<A, B, C> {
    FnNumber3("NORMINV", p, mean, std_dev)
}

///  PEARSON returns the Pearson correlation coefficient of two data sets
#[inline]
pub fn pearson<A: Array, B: Array>(independend_values: A, dependend_values: B) -> FnNumber2<A, B> {
    FnNumber2("PEARSON", independend_values, dependend_values)
}

///  Calculates the x-th sample percentile among the values in range.
#[inline]
pub fn percentile<A: Sequence, B: Number>(data: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("PERCENTILE", data, x)
}

///  Returns the percentage rank of a value in a sample.
#[inline]
pub fn percentrank<A: Sequence, B: Number, C: Number>(data: A, x: B, significance: Option<C>) -> FnNumber3<A, B, Option<C>> {
    FnNumber3("PERCENTRANK", data, x, significance)
}

///  returns the number of permutations of k objects taken from n objects
#[inline]
pub fn permut<A: Number, B: Number>(n: A, k : B) -> FnNumber2<A, B> {
    FnNumber2("PERMUT", n, k )
}

///  Returns the number of permutations for a given number of objects (repetition allowed).
#[inline]
pub fn permutationa<A: Number, B: Number>(total: A, chosen: B) -> FnNumber2<A, B> {
    FnNumber2("PERMUTATIONA", total, chosen)
}

///  Returns the values of the density function for a standard normal distribution.
#[inline]
pub fn phi<A: Number>(n: A) -> FnNumber1<A> {
    FnNumber1("PHI", n)
}

///  returns the probability or the cumulative distribution function for the Poisson distribution
#[inline]
pub fn poisson_<A: Number, B: Number, C: Logical>(x: A, lambda: B, cumulative: C) -> FnNumber3<A, B, C> {
    FnNumber3("POISSON", x, lambda, cumulative)
}

///  returns the probability or the cumulative distribution function for the Poisson distribution
#[inline]
pub fn poisson<A: Number, B: Number>(x: A, lambda: B) -> FnNumber2<A, B> {
    FnNumber2("POISSON", x, lambda)
}

///  Returns the probability that a discrete random variable lies between two limits.
#[inline]
pub fn prob<A: Array, B: Array, C: Number, D: Number>(data: A, probability: B, start: C, end: Option<D>) -> FnNumber4<A, B, C, Option<D>> {
    FnNumber4("PROB", data, probability, start, end)
}

///  Returns a quartile of a set of data points.
#[inline]
pub fn quartile<A: Sequence, B: Number>(data: A, quart: B) -> FnNumber2<A, B> {
    FnNumber2("QUARTILE", data, quart)
}

///  Returns the rank of a number in a list of numbers.
#[inline]
pub fn rank<A: Number, B: Sequence, C: Number>(value: A, data: B, order: Option<C>) -> FnNumber3<A, B, Option<C>> {
    FnNumber3("RANK", value, data, order)
}

///  Returns the square of the Pearson product moment correlation coefficient.
#[inline]
pub fn rsq<A: Array, B: Array>(array_y: A, array_x: B) -> FnNumber2<A, B> {
    FnNumber2("RSQ", array_y, array_x)
}

///  Estimates the skewness of a distribution using a sample set of numbers.
#[inline]
pub fn skew<A: Sequence>(sample: A) -> FnNumber1<A> {
    FnNumber1("SKEW", sample)
}

///  Calculates the skewness of a distribution using the population of a random variable.
#[inline]
pub fn skewp<A: Sequence>(population: A) -> FnNumber1<A> {
    FnNumber1("SKEWP", population)
}

///  Calculates the slope of the linear regression line.
#[inline]
pub fn slope<A: Array, B: Array>(year: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("SLOPE", year, x)
}

///  Finds the nth smallest value in a list.
#[inline]
pub fn small<A: Sequence, B: Number>(list: A, n: B) -> FnNumber2<A, B> {
    FnNumber2("SMALL", list, n)
}

///  Calculates a normalized value of a random variable.
#[inline]
pub fn standardize<A: Number, B: Number, C: Number>(value: A, mean: B, sigma: C) -> FnNumber3<A, B, C> {
    FnNumber3("STANDARDIZE", value, mean, sigma)
}

///  Compute the sample standard deviation of a set of numbers
#[inline]
pub fn stdev<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("STDEV", n)
}

///  Calculate the standard deviation using a sample set of values, including values of type Text and Logical.
#[inline]
pub fn stdeva<A: Any>(sample: A) -> FnNumber1<A> {
    FnNumber1("STDEVA", sample)
}

///  Calculates the standard deviation using the population of a random variable, including
#[inline]
pub fn stdevp<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("STDEVP", n)
}

///  Calculates the standard deviation using the population of a random variable, including
#[inline]
pub fn stdevpa<A: Any>(sample: A) -> FnNumber1<A> {
    FnNumber1("STDEVPA", sample)
}

///  Calculates the standard error of the predicted y value for each x in the regression.
#[inline]
pub fn steyx<A: Array, B: Array>(measured_y: A, x: B) -> FnNumber2<A, B> {
    FnNumber2("STEYX", measured_y, x)
}

///  Calculates the inverse of the two-tailed t-distributio
#[inline]
pub fn tinv<A: Number, B: Number>(probability: A, degrees_of_freedom: B) -> FnNumber2<A, B> {
    FnNumber2("TINV", probability, degrees_of_freedom)
}

///  Returns predicted values based on a simple or multiple linear regression.
#[inline]
pub fn trend_<A: Array, B: Array, C: Logical>(known_y: A, known_x: Option<B>, const_: C) -> FnArray3<A, Option<B>, C> {
    FnArray3("TREND", known_y, known_x, const_)
}

///  Returns predicted values based on a simple or multiple linear regression.
#[inline]
pub fn trend<A: Array, B: Array>(known_y: A, known_x: Option<B>) -> FnArray2<A, Option<B>> {
    FnArray2("TREND", known_y, known_x)
}

///  Returns the mean of a data set, ignoring a proportion of high and low values.
#[inline]
pub fn trimmean<A: Sequence, B: Number>(data_set: A, cut_off_fraction: B) -> FnNumber2<A, B> {
    FnNumber2("TRIMMEAN", data_set, cut_off_fraction)
}

///  Calculates the p-value of a 2-sample t-test
#[inline]
pub fn ttest<A: Array, B: Array, C: Number, D: Number>(x: A, y: B, tails: C, type_: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("TTEST", x, y, tails, type_)
}

///  Compute the sample variance of a set of numbers.
#[inline]
pub fn var<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("VAR", n)
}

///  Estimates the variance using a sample set of values, including values of type Text and
#[inline]
pub fn vara<A: Any>(sample: A) -> FnNumber1<A> {
    FnNumber1("VARA", sample)
}

///  Compute the variance of the set for a set of numbers
#[inline]
pub fn varp<A: Sequence>(n: A) -> FnNumber1<A> {
    FnNumber1("VARP", n)
}

///  Calculates the variance using the population of the distribution, including values of type Text and Logical.
#[inline]
pub fn varpa<A: Any>(sample: A) -> FnNumber1<A> {
    FnNumber1("VARPA", sample)
}

///  Calculates the Weibull distribution.
#[inline]
pub fn weibull<A: Number, B: Number, C: Number, D: Logical>(value: A, shape: B, scale: C, cumulative: D) -> FnNumber4<A, B, C, D> {
    FnNumber4("WEIBULL", value, shape, scale, cumulative)
}

///  Calculates the probability of observing a sample mean as large or larger than the mean of the given sample for samples drawn from a normal distribution.
#[inline]
pub fn ztest<A: Sequence, B: Number, C: Number>(sample: A, mean: B, sigma: Option<C>) -> FnNumber3<A, B, Option<C>> {
    FnNumber3("ZTEST", sample, mean, sigma)
}
