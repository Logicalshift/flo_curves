# Some notes on coding style in FlowBetween and related packages

One of the things I do with personal projects (and for now at least, that's what FlowBetween is to me) is experiment with different ways of doing things, especially things that are quite radically different. Some of these things are apparent in FlowBetween and its related libraries, and sometimes they're more or less noticeable. Generally things that are obvious are things that have proved beneficial enough to me that they were worth keeping up the effort.

For the most part the style is 'normal Rust with some exceptions'. In general I think my policy with PRs and such will be that style can be fixed later as it's best treated as a 'squeaky wheel' kind of thing and far too easy to wind up spending a lot of time thinking about something that doesn't really matter all that much.

## Unusual stylistic things found in FlowBetween

 * Libraries designed to work independently of the structure as a whole.
 * Command streams instead of function calls for the larger APIs
 * Order of operations over locking for concurrency
 * The only bad comment is the one that was needed but left out
 * Whitespace to create 'table alignments'

Those last two are probably the most contentious, though really they shouldn't be.

## Why be different?

Really, the question is why not use a mechanical format with a tool like rustfmt? Isn't that easier?

Sometimes easier isn't the point. The problem with a mechanical styling is that it doesn't take the reader into account at all: we don't yet have software sophisticated enough to empathize with a reader and highlight what's important about an algorithm or try to show how different pieces of data flow through it. All these tools know how to do is change whitespace to match the structure of the language. This takes away a tool that can be used to communicate meaning, and programming languages are already a very hard medium to use to communicate in.

The advantage of mechanical formatting is that it's standardised. Traits like 'readability' are inherently subjective and there's something about the software developer mindset that assumes that everything can be objectively boiled down to an algorithm, so the idea that something that looks good to one developer might not look so good to another developer is anathema and the source of many arguments. 

Picking a mechanical tool as the only means of formatting means giving up on the idea that code can look better than it does as a trade-off for fewer disputes about which format looks best. This choice would be along the grain for most commercial software, but FlowBetween is built around the conceit that vector editing and animation software can be much better than they are, something that extends to the layout of the codebase itself.

It's always possible that this just makes things worse, but that isn't the goal: none of this is done to spite anyone.

So the idea is this: FlowBetween starts with what rustfmt does but is fearless about changing that when a different format seems like it might be clearer. I'm hopeful that discussions can be steered to why particular approaches are better or worse rather than trying to follow the argumentative approach of assuming things are either right or wrong.

## Whitespace to create 'table alignments'

This is probably the most contraversial choice, as it really fights what the existing tooling does and is prone to getting wiped out. When done well this can highlight structures and dependencies that are otherwise difficult to pick out at a glance.

The basic idea is that tables are a very readable way to present information, so that this:

```Rust
pub struct BrushPreview {
    current_brush:          Arc<dyn Brush>,
    brush_properties:       BrushProperties,
    raw_points:             Vec<RawPoint>,
    brush_points:           Option<Arc<Vec<BrushPoint>>>,
    combined_element:       Option<Vector>
}
```

is much more readable than this:

```Rust
pub struct BrushPreview {
    current_brush: Arc<dyn Brush>,
    brush_properties: BrushProperties,
    raw_points: Vec<RawPoint>,
    brush_points: Option<Arc<Vec<BrushPoint>>>,
    combined_element: Option<Vector>
}
```

because the types and the names are visually separated. This seems like it should be uncontroversial: a formatted table is much more readable than a CSV file for the same reason.

This extends to initialising stuff too:

```Rust
        let mut edit_publisher  = animation.edit();
        let animation           = Arc::new(animation);
        let tools               = ToolModel::new();
        let timeline            = TimelineModel::new(Arc::clone(&animation), edit_publisher.subscribe());
        let frame_edit_counter  = bind(0);
        let frame               = FrameModel::new(Arc::clone(&animation), edit_publisher.subscribe(), BindRef::new(&timeline.current_time), BindRef::new(&frame_edit_counter), BindRef::new(&timeline.selected_layer));
        let selection           = SelectionModel::new(Arc::clone(&animation), &frame, &timeline);
        let onion_skin          = OnionSkinModel::new(Arc::clone(&animation), &timeline);
        let sidebar             = SidebarModel::new();
```

beats

```Rust
        let mut edit_publisher = animation.edit();
        let animation = Arc::new(animation);
        let tools = ToolModel::new();
        let timeline = TimelineModel::new(Arc::clone(&animation), edit_publisher.subscribe());
        let frame_edit_counter = bind(0);
        let frame = FrameModel::new(Arc::clone(&animation), edit_publisher.subscribe(), BindRef::new(&timeline.current_time), BindRef::new(&frame_edit_counter), BindRef::new(&timeline.selected_layer));
        let selection = SelectionModel::new(Arc::clone(&animation), &frame, &timeline);
        let onion_skin = OnionSkinModel::new(Arc::clone(&animation), &timeline);
        let sidebar = SidebarModel::new();
```

for the same reason: picking out the variable names and their assignments is much easier, because they're visually separate.

It's probably worth noting that this kind of visual separation is the important thing rather than the specifics: it's not something that formatting tools can really deal with as they just blindly enforce rules without considering that the purpose of any formatting is to help make the important parts of the code visible.

## The only bad comment is the one that was needed but left out

Comments are also controversial. I disagree strongly with the prevailing opinion in the community that code can or should be entirely self-documenting, as I don't think I've ever really encountered any code outside of demonstrates written to make a point where that has actually occurred.

Essentially, I think the idea of documenting code boils down to two possible worlds: one where meaning is sometimes lost because of missing documentation, or one where sometimes the documentation says something too obvious. FlowBetween tries to choose the first world where possible as really an unnecessary comment harms nothing but the misunderstanding that can result from a missing explanation can waste an incredible amount of time.

(Raging about obvious comments also wastes time, but this is really a matter of personal choice: go ahead if you think it's a valuable use of your time, but do try to respect the time of others while you do so)

Generally the code documentation should be improved over time: it's usually possible to improve 'obvious' comments by finding something less obvious to talk about rather than removing them. Generally the reason they're noticeable in the first place is that they were in the place of some other piece of information you were looking for, which makes improving them quite easy.

## Order of operations over locking for concurrency

FlowBetween has the `desync` library for this purpose, and tries to use it instead of locks for a lot of its concurrency purposes. This makes a lot of the typical concurrency problems disappear: `desync` performs operations in the order that they're queued up and is much less prone to race conditions as a result. It also prevents the 'spooky action at a distance' effect that can arise with threads (where you send a message to another piece of code far away in the codebase that's hard to find) by making it easy to keep asynchronous cause-and-effect code in the same place.

Specifically for Rust, `desync` is great because it makes some issues with lifetimes and multi-threaded code (and lifetimes and futures) just evaporate.

## Command streams instead of function calls

Sending commands to a component using a stream has a big disadvantage: return values are a bit of a pain. Streams are also a bit more work than an API designed to use function calls.

However, streams have some advantages: they make it easy to intercept, change and process commands en masse. They're great for debugging odd behaviours. They're really good at cutting down dependencies by providing a strong barrier between different parts of the code base.

## Libraries designed to work independently of the structure as a whole.

Hopefully not controversial but still something that needs to be considered in the design for things. A lot of the libraries designed for FlowBetween, like `flo_curves`, the various rendering libraries, even the main animation library `flo_canvas_animation` could easily have been designed in a way that made them not easily separable from the other components of the tool.

This is mostly a question of dependency management: if something can be designed so that it returns a value instead of making a callback, it is less tightly coupled. If something can use a less specific data type, it is less tightly coupled. Less tightly coupled code is easier to re-use in new contexts.

Making the components of the tool easier to use independently of one another makes it easier to use for more imaginative purposes. (For example, generating animation files from scratch, or using `flo_canvas_animation` to generate dynamic animations in a game or even another application)
