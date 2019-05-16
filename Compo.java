/*
 * This is me testing function composition in Java.
 *
 */
import java.util.List;
import java.util.stream.Stream;
import java.util.stream.Collectors;
import java.util.function.Function;
import java.util.function.Consumer;

public class Compo {
	public static void main(String[] argv) {
		System.out.println("Starting up...");

		Doubler doubler = new Doubler();
		PlusTwo plusTwo = new PlusTwo();
		Printer printer = new Printer(doubler.andThen(plusTwo));
		// ^ for the awesomer version, passing the Function to
		//   the constructor wouldn't be necessary.

		List<Long> result = Stream.of(2L, 4L, 5L, 7L)
			// this would be awesomer
			// .peek(doubler.andThen(plusTwo).andThen(printer))
			.peek(printer)
			.collect(Collectors.toList());
		System.out.println("Result: " + result);

		ToString toString = new ToString();
		ToInteger toInteger = new ToInteger();

		Function<Long, Integer> f = toString.andThen(toInteger);
		System.out.println("Conversion: " + f.apply(12L));
	}

	private static class Doubler implements Function<Long, Long> {
		@Override
		public Long apply(Long input) {
			return input * 2;
		}
	}

	private static class PlusTwo implements Function<Long, Long> {
		@Override
		public Long apply(Long input) {
			return input + 2L;
		}
	}

	private static class Printer implements Consumer<Long> {

		Function<Long, Long> processor;

		public Printer(Function<Long, Long> processor) {
			this.processor = processor;
		}

		@Override
		public void accept(Long input) {
			System.out.println("Consumer: " + processor.apply(input));
		}
	}

	private static class ToString implements Function<Long, String> {
		@Override
		public String apply(Long input) {
			return input.toString();
		}
	}

	private static class ToInteger implements Function<String, Integer> {
		@Override
		public Integer apply(String input) {
			return Integer.parseInt(input);
		}
	}
}
